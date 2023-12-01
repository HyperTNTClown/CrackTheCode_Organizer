import { CanActivateFn } from '@angular/router';
import {inject} from "@angular/core";
import {ApiService} from "./api.service";

export const adminGuardGuard: CanActivateFn = async (route, state) => {
  // Debugging
  return true;
  const response = await inject(ApiService).valid_admin();
  console.log(response);
  return response.valid;
};
