import {HttpClient} from '@angular/common/http';
import {Injectable} from '@angular/core';

@Injectable({providedIn: 'root'})
export class TeamspeakService {
  constructor(private http: HttpClient) {}

  getTeamspeakData() {
    return this.http.get('/api/tsquery');
  }

  getTeamspeakConfig() {
    return this.http.get('/api/config');
  }
}
