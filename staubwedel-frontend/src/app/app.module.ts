import {NgModule} from '@angular/core';
import {FlexLayoutModule} from '@angular/flex-layout';
import {BrowserModule} from '@angular/platform-browser';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {HomeComponent} from './home/home.component';
import {MaterialModule} from './material/material.module';
import {NavbarComponent} from './navbar/navbar.component';
import {TeamspeakComponent} from './teamspeak/teamspeak.component';
import {TwitchComponent} from './twitch/twitch.component';

@NgModule({
  declarations: [
    AppComponent, HomeComponent, NavbarComponent, TeamspeakComponent,
    TwitchComponent
  ],
  imports: [
    BrowserModule, AppRoutingModule, BrowserAnimationsModule, FlexLayoutModule,
    MaterialModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
