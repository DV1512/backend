CREATE provider:Email;
CREATE provider:Google;
CREATE provider:Github;

CREATE provider_config:Google SET
        user_info_url = 'https://www.googleapis.com/oauth2/v3/userinfo',
        auth_url = 'https://accounts.google.com/o/oauth2/auth',
        token_url = 'https://oauth2.googleapis.com/token',
        scopes = [
            'https://www.googleapis.com/auth/userinfo.email',
            'https://www.googleapis.com/auth/userinfo.profile',
            'openid'
        ],
        redirect_endpoint = '/api/v1/oauth/google/callback'
;

UPDATE provider:Google SET config = provider_config:Google;

CREATE provider_config:Github SET
        user_info_url = 'https://api.github.com/user',
        auth_url = 'https://github.com/login/oauth/authorize',
        token_url = 'https://github.com/login/oauth/access_token',
        scopes = [
            'user:email'
        ],
        redirect_endpoint = '/api/v1/oauth/github/callback'
;

UPDATE provider:Github SET config = provider_config:Github;