import {HttpClientModule} from '@angular/common/http';
import {NgModule} from '@angular/core';
import {FlexLayoutModule} from '@angular/flex-layout';
import {BrowserModule} from '@angular/platform-browser';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {ClipboardModule} from 'ngx-clipboard';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {BlogComponent} from './blog/blog.component';
import {HomeComponent} from './home/home.component';
import {MaterialModule} from './material/material.module';
import {NavbarComponent} from './navbar/navbar.component';
import {TeamspeakComponent} from './teamspeak/teamspeak.component';
import {TwitchChatComponent} from './twitch-chat/twitch-chat.component';
import {TwitchVideoComponent} from './twitch-video/twitch-video.component';

@NgModule({
  declarations: [
    AppComponent, HomeComponent, NavbarComponent, TeamspeakComponent,
    TwitchVideoComponent, BlogComponent, TwitchChatComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    FlexLayoutModule,
    MaterialModule,
    HttpClientModule,
    ClipboardModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
