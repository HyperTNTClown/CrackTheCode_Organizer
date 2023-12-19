import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterModule, Routes} from "@angular/router";

const routes: Routes = [
  {
    path: '',
    loadComponent: () => import('./admin.component').then(m => m.AdminComponent),
    children: [
      {
        path: 'dashboard',
        loadChildren: () => import('./dashboard/dashboard.module').then(m => m.DashboardModule)
      },
      {
        path: 'puzzle-set-create',
        loadChildren: () => import('./create/puzzle-set/puzzle-set.module').then(m => m.PuzzleSetModule)
      },
      {
        path: 'puzzle',
        loadComponent: () => import('./list/list.component').then(m => m.ListComponent)
      },
      {
        path: 'puzzle-create',
        loadChildren: () => import('./create/puzzle/puzzle.module').then(m => m.PuzzleModule)
      },
      {
        path: 'puzzle-edit/:id',
        loadComponent: () => import('./edit/edit.component').then(m => m.EditComponent)
      },
      {
        path: 'schedule',
        loadChildren: () => import('./schedule/schedule.module').then(m => m.ScheduleModule)
      }
    ]
  }
];

@NgModule({
            declarations: [],
            imports: [
              CommonModule,
              RouterModule.forChild(routes)
            ],
            exports: [RouterModule]
          })
export class AdminModule {
}
