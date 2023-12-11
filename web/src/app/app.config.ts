import {ApplicationConfig, importProvidersFrom} from '@angular/core';
import {provideRouter} from '@angular/router';

import {routes} from './app.routes';
import {
  HttpClient,
  provideHttpClient,
  withFetch,
  withInterceptors
} from "@angular/common/http";
import {provideAnimations} from '@angular/platform-browser/animations';
import {provideMarkdown} from "ngx-markdown";
import {AngularMarkdownEditorModule} from "angular-markdown-editor";

export const appConfig: ApplicationConfig = {
    providers: [
        provideRouter(routes),
        provideHttpClient(withFetch(), withInterceptors([])),
        provideAnimations(),
        provideMarkdown({loader: HttpClient}),
        importProvidersFrom(AngularMarkdownEditorModule.forRoot({
            savable: true,
        }))
    ]
};
