import {Routes} from '@angular/router';
import {BaseComponent} from './base/base.component';
import {AuthComponent} from './auth/auth.component';
import {adminGuardGuard} from "./admin-guard.guard";

export const routes: Routes = [
  {
    path: '',
    component: BaseComponent
  },
  {
    path: 'auth',
    component: AuthComponent
  },
  {
    path: 'admin',
    //loadComponent: () => import('./admin/admin.component').then(m => m.AdminComponent),
    canActivate: [adminGuardGuard],
    /*children: [
      {
        path: 'puzzle-sets',
        loadComponent: () => import('./admin/create/puzzle-set/puzzle-set.component').then(m => m.PuzzleSetComponent),
      }
    ]*/
    loadChildren: () => import('./admin/admin.module').then(m => m.AdminModule)
  }
];
