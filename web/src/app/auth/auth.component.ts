import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule} from "@angular/forms";
import {ApiService} from "../api.service";
import {ActivatedRoute, Router} from "@angular/router";
import {firstValueFrom} from "rxjs";
import {MatInputModule} from "@angular/material/input";
import {MatButtonModule} from "@angular/material/button";

@Component({
    selector: 'app-auth',
    standalone: true,
    imports: [CommonModule, ReactiveFormsModule, FormsModule, MatInputModule, MatButtonModule],
    providers: [ApiService, Router],
    templateUrl: './auth.component.html',
    styleUrl: './auth.component.css'
})
export class AuthComponent implements OnInit {

    constructor(private apiService: ApiService, private route: ActivatedRoute, private router: Router) {
    }

    public admin = false;


    form = new FormGroup({
        email: new FormControl(),
        password: new FormControl()
    });

    ngOnInit(): void {
        console.log('auth component initialized');
        //this.login();
        this.form.controls.email.valueChanges.subscribe((value) => {
            this.apiService.is_admin(value).subscribe((response) => {
                console.log(response);
                // @ts-ignore
                this.admin = response['is_admin'];
            })
        })
    }

    submit(ev: SubmitEvent) {
        switch ((ev.submitter as HTMLInputElement).value.toLowerCase()) {
            case 'login':
                this.apiService.login(this.form.controls.email.value, this.form.controls.password.value).subscribe((response) => {
                    console.log(response);
                })
                break;
            case 'register':
                firstValueFrom(this.apiService.register(this.form.controls.email.value, this.form.controls.password.value))
                    .then((response) => {
                        console.log(response + " register");
                    })
                break;
            case 'admin login':
                firstValueFrom(this.apiService.login(this.form.controls.email.value, this.form.controls.password.value)).then((response) => {
                    console.log(response);
                    this.router.navigate(['/admin']);
                })
                break;
        }
    }
}
