import {CommonModule} from '@angular/common';
import {NgModule} from '@angular/core';
import {MatToolbarModule, MatTreeModule} from '@angular/material';

@NgModule({
  declarations: [],
  imports: [CommonModule, MatToolbarModule, MatTreeModule],
  exports: [MatToolbarModule, MatTreeModule],
})
export class MaterialModule {
}
