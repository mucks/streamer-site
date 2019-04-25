import {NestedTreeControl} from '@angular/cdk/tree';
import {Component, OnInit} from '@angular/core';
import {MatTreeNestedDataSource} from '@angular/material';

interface ChannelNode {
  name: string;
  children?: ChannelNode[];
  clients?: ChannelNode[];
}

const TREE_DATA: ChannelNode[] = [{
  name: 'channel 1',
  children: [
    {name: 'channel 2'},
    {name: 'channel 3'},
    {name: 'channel 4'},
  ],
}];


@Component({
  selector: 'app-teamspeak',
  templateUrl: './teamspeak.component.html',
  styleUrls: ['./teamspeak.component.css']
})
export class TeamspeakComponent implements OnInit {
  treeControl = new NestedTreeControl<ChannelNode>(node => node.children);
  dataSource = new MatTreeNestedDataSource<ChannelNode>();
  channels: ChannelNode[] = [];

  constructor() {}

  hasChild = (_: number, node: ChannelNode) =>
      !!node.children && node.children.length > 0;

  hasClient = (_: number, node: ChannelNode) =>
      !!node.clients && node.clients.length > 0;

  ngOnInit() {
    const socket = new WebSocket('ws://localhost:3012');

    socket.addEventListener('open', event => {
      socket.send('hello world');
    });

    socket.addEventListener('message', event => {
      const channels = JSON.parse(event.data);
      console.log(channels);
      this.dataSource.data = channels;
    });
  }
}
