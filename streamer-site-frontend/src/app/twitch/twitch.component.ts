import {HttpClient} from '@angular/common/http';
import {Component, OnInit} from '@angular/core';
import {DomSanitizer, SafeResourceUrl} from '@angular/platform-browser';

@Component({
  selector: 'app-twitch',
  templateUrl: './twitch.component.html',
  styleUrls: ['./twitch.component.css']
})
export class TwitchComponent implements OnInit {
  videoHeight: number;
  twitchVideoUrl: SafeResourceUrl;
  twitchChatUrl: SafeResourceUrl;
  twitchId: string;

  constructor(private http: HttpClient, private sanitizer: DomSanitizer) {}

  ngOnInit() {
    this.http.get('/api/config').subscribe(data => {
      const channel = data['twitch_channel'];
      this.twitchId = channel;
      this.twitchVideoUrl = this.sanitizer.bypassSecurityTrustResourceUrl(
          `https://player.twitch.tv/?channel=${channel}`);
      this.twitchChatUrl = this.sanitizer.bypassSecurityTrustResourceUrl(
          `https://www.twitch.tv/embed/${channel}/chat?darkpopout`);
    });
  }
}
