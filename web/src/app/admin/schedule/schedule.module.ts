import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterModule, Routes} from "@angular/router";
import {ScheduleComponent} from "./schedule.component";

const routes: Routes = [
  {
    path: '',
    component: ScheduleComponent
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
export class ScheduleModule {
}
