import {NestedTreeControl} from '@angular/cdk/tree';
import {Component, OnInit} from '@angular/core';
import {MatTreeNestedDataSource} from '@angular/material';

interface ChannelNode {
  name: string;
  children?: ChannelNode[];
  is_client: boolean,
}

@Component({
  selector: 'app-teamspeak',
  templateUrl: './teamspeak.component.html',
  styleUrls: ['./teamspeak.component.css']
})
export class TeamspeakComponent implements OnInit {
  treeControl = new NestedTreeControl<ChannelNode>(node => node.children);
  dataSource = new MatTreeNestedDataSource<ChannelNode>();

  constructor() {}

  hasChild = (_: number, node: ChannelNode) =>
      !!node.children && node.children.length > 0;

  ngOnInit() {
    const socket = new WebSocket('ws://localhost:3012');

    socket.addEventListener('open', event => {
      setInterval(() => {
        socket.send('hello world');
      }, 1000);
    });

    socket.addEventListener('message', event => {
      const output = JSON.parse(event.data);
      if (output.status == 200) {
        this.dataSource.data = output.nodes;
        this.treeControl.dataNodes = output.nodes;
        this.treeControl.expandAll();
      }
    });
  }

  formatName(name: string): string {
    name = name.replace('\\s', ' ');
    name = name.replace('\\s', ' ');
    return name;
  }
}
