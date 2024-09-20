CREATE provider:Email;
CREATE provider:Google;

CREATE provider_config:Google SET
        user_info_url = 'https://www.googleapis.com/oauth2/v3/userinfo',
        auth_url = 'https://accounts.google.com/o/oauth2/auth',
        token_url = 'https://oauth2.googleapis.com/token',
        scopes = [
            'https://www.googleapis.com/auth/userinfo.email',
            'https://www.googleapis.com/auth/userinfo.profile',
            'openid'
        ],
        redirect_endpoint = '/oauth/google/callback'
;

UPDATE provider:Google SET config = provider_config:Google;