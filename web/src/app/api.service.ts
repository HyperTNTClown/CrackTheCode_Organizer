import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {firstValueFrom} from "rxjs";
import {fromFetch} from "rxjs/internal/observable/dom/fetch";

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
    return fromFetch(url)
  }

  private post(url: string, data: any) {
    return fromFetch(url, {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    })
  }

  private put(url: string, data: any) {
    return fromFetch(url, {
      method: 'PUT',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    })
  }

  public async is_admin(username: string): Promise<{ is_admin: boolean }> {
    const response = await firstValueFrom(this.get(`/api/v1/is-admin?email=${username}`));
    return await response.json() as { is_admin: boolean };
  }

  public login(email: string, password: string) {
    return fromFetch(`/api/v1/auth/login`, {
      method: 'POST',
      body: JSON.stringify({email, password}),
      headers: {
        'Content-Type': 'application/json'
      }
    })
  }

  public register(email: string, password: string) {
    return this.post(`/api/v1/auth/register`, {email, password});
  }

  async fetch_sets(): Promise<PuzzleSet[]> {
    const response = await firstValueFrom(this.get(`/api/v1/admin/fetch`));
    return await response.json() as PuzzleSet[];
  }

  async valid_admin(): Promise<{ valid: boolean, id: string }> {
    const response = await firstValueFrom(this.get(`/api/v1/auth/valid-admin`));
    return await response.json() as { valid: boolean, id: string };
  }

  async create_set(name: string, description: string): Promise<PuzzleSet> {
    const response = await firstValueFrom(this.put(`/api/v1/admin/puzzle-set`, {name, description}));
    return await response.json() as PuzzleSet;
  }
}
