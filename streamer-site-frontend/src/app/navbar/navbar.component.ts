import {HttpClient} from '@angular/common/http';
import {Component, OnInit} from '@angular/core';
import {Title} from '@angular/platform-browser';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.css']
})
export class NavbarComponent implements OnInit {
  title = 'Streamer Site';

  constructor(private http: HttpClient, private titleService: Title) {}

  ngOnInit() {
    this.http.get('/api/config').subscribe(data => {
      this.title = data['page_title'];
      this.titleService.setTitle(this.title);
    });
  }
}
