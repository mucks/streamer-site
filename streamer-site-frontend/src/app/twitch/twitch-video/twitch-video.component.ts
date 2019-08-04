import {Component, EventEmitter, HostListener, Input, OnInit, Output} from '@angular/core';
import {SafeResourceUrl} from '@angular/platform-browser';

@Component({
  selector: 'app-twitch-video',
  templateUrl: './twitch-video.component.html',
  styleUrls: ['./twitch-video.component.css']
})
export class TwitchVideoComponent implements OnInit {
  videoWidth: number;
  videoHeight: number;

  @Output() heightChanged = new EventEmitter<number>();
  @Input() twitchUrl: SafeResourceUrl;

  constructor() {
    this.twitchSize();
  }

  ngOnInit() {}

  @HostListener('window:resize', ['$event'])
  onResize() {
    this.twitchSize();
  }

  twitchSize() {
    const width = window.innerWidth;

    if (width < 400) {
      this.videoWidth = window.innerWidth;
      this.videoHeight = 240;
    } else if (width > 400 && width < 640) {
      this.videoWidth = window.innerWidth;
      this.videoHeight = 240;
    } else if (width > 640 && width < 1280) {
      this.videoWidth = 640;
      this.videoHeight = 480;
    } else if (width > 1280 && width < 1650) {
      this.videoWidth = 800;
      this.videoHeight = 600;
    } else if (width > 1650 && width < 2560) {
      this.videoWidth = 1280;
      this.videoHeight = 720;
    } else if (width > 2560) {
      this.videoWidth = 1920;
      this.videoHeight = 1080;
    }
    this.heightChanged.emit(this.videoHeight);
  }
}
