import {CommonModule} from '@angular/common';
import {NgModule} from '@angular/core';
import {MatButtonModule, MatCardModule, MatIconModule, MatToolbarModule, MatTreeModule} from '@angular/material';

@NgModule({
  declarations: [],
  imports: [
    CommonModule, MatToolbarModule, MatTreeModule, MatCardModule, MatIconModule,
    MatButtonModule
  ],
  exports: [
    MatToolbarModule, MatTreeModule, MatCardModule, MatIconModule,
    MatButtonModule
  ],
})
export class MaterialModule {
}
