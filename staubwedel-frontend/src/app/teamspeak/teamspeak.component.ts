import {NestedTreeControl} from '@angular/cdk/tree';
import {Component, OnDestroy, OnInit} from '@angular/core';
import {MatTreeNestedDataSource} from '@angular/material';

import {TeamspeakService} from './teamspeak.service';

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
export class TeamspeakComponent implements OnInit, OnDestroy {
  treeControl = new NestedTreeControl<ChannelNode>(node => node.children);
  dataSource = new MatTreeNestedDataSource<ChannelNode>();
  interval: any;
  teamspeakUrl = 'derstaubwedel.com';

  constructor(private tsService: TeamspeakService) {}

  hasChild = (_: number, node: ChannelNode) =>
      !!node.children && node.children.length > 0;

  ngOnInit() {
    this.applyTeamspeak();
    this.interval = setInterval(() => {
      this.applyTeamspeak();
    }, 1000);
  }

  applyTeamspeak() {
    this.tsService.getTeamspeakData().subscribe(data => {
      if (data['status'] == 200) {
        this.dataSource.data = data['nodes'];
        this.treeControl.dataNodes = data['nodes'];
        this.treeControl.expandAll();
      }
    });
  }

  ngOnDestroy() {
    clearInterval(this.interval);
  }

  formatName(name: string): string {
    name = name.replace('\\s', ' ');
    name = name.replace('\\s', ' ');
    return name;
  }
}
