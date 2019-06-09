import {Component, Input, OnInit} from '@angular/core';

@Component({
  selector: 'app-twitch-chat',
  templateUrl: './twitch-chat.component.html',
  styleUrls: ['./twitch-chat.component.css']
})
export class TwitchChatComponent implements OnInit {
  @Input() chatHeight: number;

  constructor() {
    this.chatHeight = 720;
  }

  ngOnInit() {}
}
