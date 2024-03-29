import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule, Routes} from "@angular/router";
import {PuzzleComponent} from "./puzzle.component";

const routes: Routes = [
  {
    path: '',
    component: PuzzleComponent
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
export class PuzzleModule {
}
