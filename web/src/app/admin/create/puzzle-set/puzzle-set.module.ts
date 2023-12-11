import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule, Routes} from "@angular/router";
import {PuzzleSetComponent} from "./puzzle-set.component";

const routes: Routes = [
  {
    path: '',
    component: PuzzleSetComponent
  }
]

@NgModule({
            declarations: [],
            imports: [
              CommonModule,
              RouterModule.forChild(routes)
            ],
            exports: [RouterModule]
          })
export class PuzzleSetModule {
}
