import {NestedTreeControl} from '@angular/cdk/tree';
import {Component, OnInit} from '@angular/core';

interface ChannelNode {
  name: string;
  children?: ChannelNode[]
}

@Component({
  selector: 'app-teamspeak',
  templateUrl: './teamspeak.component.html',
  styleUrls: ['./teamspeak.component.css']
})
export class TeamspeakComponent implements OnInit {
  treeControl = new NestedTreeControl<ChannelNode>(node => node.children);
  constructor() {}

  ngOnInit() {
    const socket = new WebSocket('ws://localhost:3012');

    socket.addEventListener('open', event => {
      socket.send('hello world');
    });

    socket.addEventListener('message', event => {
      console.log('message from server ', event.data);
    });
  }
}
