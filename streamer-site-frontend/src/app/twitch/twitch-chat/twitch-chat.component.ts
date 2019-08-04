import {Component, Input, OnInit} from '@angular/core';
import {SafeResourceUrl} from '@angular/platform-browser';

@Component({
  selector: 'app-twitch-chat',
  templateUrl: './twitch-chat.component.html',
  styleUrls: ['./twitch-chat.component.css']
})
export class TwitchChatComponent implements OnInit {
  @Input() chatHeight: number;
  @Input() twitchId: string;
  @Input() twitchUrl: SafeResourceUrl;

  constructor() {
    this.chatHeight = 720;
  }

  ngOnInit() {}
}
