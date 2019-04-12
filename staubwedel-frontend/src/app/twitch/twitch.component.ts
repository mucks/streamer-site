import {Component, OnInit} from '@angular/core';

declare const Twitch: any;

@Component({
  selector: 'app-twitch',
  templateUrl: './twitch.component.html',
  styleUrls: ['./twitch.component.css']
})
export class TwitchComponent implements OnInit {
  constructor() {}

  getTwitchUrl() {}

  ngOnInit() {
    let options = {
      'width': 1280,
      'height': 720,
      'channel': 'derstaubwedel',
      'theme': 'dark',
    };

    // let twitch = new Twitch();
    let player = new Twitch.Embed('twitch-embed', options);
  }
}
