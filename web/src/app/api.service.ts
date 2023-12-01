import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {firstValueFrom, Observable} from "rxjs";

export type PuzzleSet = {
  id: number,
  name: string,
  description: string,
  created: Date,
  modified: Date,
}

@Injectable({
  providedIn: 'root'
})
export class ApiService {

  constructor(private http: HttpClient) {
  }

  private get(url: string) {
    return this.http.get(url);
  }

  private post(url: string, data: any) {
    return this.http.post(url, data);
  }

  public is_admin(username: string): Observable<{ is_admin: boolean }> {
    return this.http.get(`/api/v1/is-admin?email=${username}`) as Observable<{ is_admin: boolean }>;
  }

  public login(email: string, password: string) {
    return this.post(`/api/v1/auth/login`, {email, password});
  }

  public register(email: string, password: string) {
    return this.post(`/api/v1/auth/register`, {email, password});
  }

  fetch_sets(): Observable<PuzzleSet[]> {
    return this.get(`/api/v1/admin/fetch`) as Observable<PuzzleSet[]>;
  }

  valid_admin() : Promise<{ valid: boolean, id: string }> {
    return firstValueFrom(this.get(`/api/v1/auth/valid-admin`) as Observable<{valid: boolean, id: string}>);
  }
}
