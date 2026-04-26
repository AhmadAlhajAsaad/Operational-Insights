import { Configuration, PopupRequest, RedirectRequest } from '@azure/msal-browser';

/**
 * Configuration object to be passed to MSAL instance on creation.
 * For a full list of MSAL.js configuration parameters, visit:
 * https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md
 */

//  BELANGRIJKE OPMERKING:
// Deze configuratie moet worden aangepast met de echte waarden van jouw Azure AD tenant
// Vraag aan Brian of de IT-afdeling om:
// 1. Client ID (Application ID)
// 2. Tenant ID (of gebruik 'organizations' voor multi-tenant)
// 3. Redirect URI (moet geregistreerd zijn in Azure Portal)

export const msalConfig: Configuration = {
  auth: {
    clientId: 'YOUR_CLIENT_ID_HERE', //  Vervang met de echte Client ID van Azure AD app registratie
    authority: 'https://login.microsoftonline.com/YOUR_TENANT_ID_HERE', //  Vervang met tenant ID
    redirectUri: window.location.origin, // Automatisch de huidige URL (development of productie)
    postLogoutRedirectUri: window.location.origin,
    navigateToLoginRequestUrl: true,
  },
  cache: {
    cacheLocation: 'sessionStorage', // "sessionStorage" is veiliger voor sensitieve data
    storeAuthStateInCookie: false, // Set to "true" if you are having issues on IE11 or Edge
  },
  system: {
    loggerOptions: {
      loggerCallback: (level, message, containsPii) => {
        if (containsPii) {
          return;
        }
        switch (level) {
          case 0: // LogLevel.Error
            console.error(message);
            return;
          case 1: // LogLevel.Warning
            console.warn(message);
            return;
          case 2: // LogLevel.Info
            console.info(message);
            return;
          case 3: // LogLevel.Verbose
            console.debug(message);
            return;
        }
      },
    },
  },
};

/**
 * Scopes you add here will be prompted for user consent during sign-in.
 * By default, MSAL.js will add OIDC scopes (openid, profile, email) to any login request.
 */
export const loginRequest: PopupRequest | RedirectRequest = {
  scopes: ['User.Read'], // Microsoft Graph API scope om basisinformatie van gebruiker op te halen
};

/**
 * Add here the scopes to request when obtaining an access token for MS Graph API
 */
export const graphConfig = {
  graphMeEndpoint: 'https://graph.microsoft.com/v1.0/me',
};
