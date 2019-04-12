import {Component, OnInit} from '@angular/core';

declare const VANTA: any;

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'Der Staubwedel';

  ngOnInit() {
    VANTA.FOG({
      el: 'body',
      highlightColor: 0x201e16,
      midtoneColor: 0x0,
      lowlightColor: 0x130e28,
      baseColor: 0xb6b1b1,
      blurFactor: 0.53
    });
  }
}
