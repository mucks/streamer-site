import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';

import {HomeComponent} from './home/home.component';
import {TeamspeakComponent} from './teamspeak/teamspeak.component';
import { BlogComponent } from './blog/blog.component';

const routes: Routes = [
  {path: '', component: HomeComponent},
  {path: 'teamspeak', component: TeamspeakComponent},
  {path: 'blog', component: BlogComponent},
];

@NgModule({imports: [RouterModule.forRoot(routes)], exports: [RouterModule]})
export class AppRoutingModule {
}
