"""Super Downloads override for yt-dlp's Vimeo extractor.

yt-dlp's default `web` client refuses to run without a logged-in session
("The web client only works when logged-in"). vimeo.com itself plays public
videos anonymously: the clip page carries a per-clip signature
(`<meta name="video-signature-<id>">`) and the API accepts it as
`anon_signature`, handing back a signed `embed_player_config_url`. That works
even for videos whose embeds are restricted to other domains, which otherwise
fail with "Cannot download embed-only video without embedding URL".

When the stock path fails that way, replay what the browser does. Any problem
in the fallback re-raises the original error, so a future yt-dlp change can
only make this plugin a no-op, never break what already worked. DRM-protected
videos are untouched: their config carries no playable streams.

Loaded by the app with `--plugin-dirs <Resources>/ytdlp-plugins`.
"""

try:
    from yt_dlp.extractor.vimeo import VimeoIE
    from yt_dlp.utils import ExtractorError, traverse_obj, url_or_none
except ImportError:  # yt-dlp layout changed: skip the override entirely
    VimeoIE = None

if VimeoIE is not None:

    class SuperDownloadsVimeoIE(VimeoIE, plugin_name='superdownloads'):

        def _extract_from_api(self, video_id, unlisted_hash=None):
            try:
                return super()._extract_from_api(video_id, unlisted_hash)
            except ExtractorError as error:
                if self._is_logged_in or 'only works when logged-in' not in str(error):
                    raise
                try:
                    info = self._extract_anonymous(video_id, unlisted_hash)
                except Exception as fallback_error:
                    self.write_debug(f'{video_id}: anonymous signature failed: {fallback_error}')
                    info = None
                if not info:
                    raise error
                return info

        def _extract_anonymous(self, video_id, unlisted_hash):
            self.write_debug(f'{video_id}: web client needs login; trying anonymous signature')
            page_url = f'https://vimeo.com/{video_id}' + (f'/{unlisted_hash}' if unlisted_hash else '')
            # Without a browser TLS fingerprint vimeo.com serves a crawler page
            # that has no signature, so this needs impersonation (curl_cffi).
            webpage = self._download_webpage(
                page_url, video_id, 'Downloading clip page', fatal=False, impersonate=True)
            signature = webpage and self._html_search_meta(
                f'video-signature-{video_id}', webpage, 'anonymous signature', default=None)
            if not signature:
                return None

            jwt = self._fetch_viewer_info(video_id)['jwt']
            api_id = f'{video_id}:{unlisted_hash}' if unlisted_hash else video_id
            video = self._download_json(
                f'https://api.vimeo.com/videos/{api_id}', video_id,
                'Downloading anonymous API JSON', fatal=False,
                headers={
                    'Authorization': f'jwt {jwt}',
                    'Accept': 'application/vnd.vimeo.*+json;version=3.4.10',
                    'Referer': page_url,
                },
                query={
                    'anon_signature': signature,
                    'fields': 'embed_player_config_url,description',
                })
            config_url = traverse_obj(video, ('embed_player_config_url', {url_or_none}))
            if not config_url:
                return None

            config = self._download_json(
                config_url, video_id, 'Downloading player config', headers={'Referer': page_url})
            info = self._parse_config(config, video_id)
            if description := traverse_obj(video, ('description', {str})):
                info.setdefault('description', description)
            info['webpage_url'] = page_url
            return info
