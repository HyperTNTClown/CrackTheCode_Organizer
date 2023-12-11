import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ApiService, PuzzleSet} from "../api.service";
import {CreateComponent} from "./create/create.component";
import {PuzzleSetComponent} from "./create/puzzle-set/puzzle-set.component";
import {RouterLink, RouterLinkActive, RouterOutlet} from "@angular/router";
import {CdkAccordionModule} from "@angular/cdk/accordion";

@Component({
  selector: 'app-admin',
  standalone: true,
             imports: [CommonModule, CreateComponent, PuzzleSetComponent, RouterOutlet, RouterLink, RouterLinkActive, CdkAccordionModule],
  providers: [ApiService],
  templateUrl: './admin.component.html',
  styleUrl: './admin.component.css'
})
export class AdminComponent implements OnInit {
  constructor(private api: ApiService) {
  }

  sets : PuzzleSet[] = []
  created: Date = new Date();

  ngOnInit(): void {
    console.log('admin component initialized');
    /*this.api.fetch_sets().then((response) => {
      console.log(response);
      let created= Date.parse(response[0].created as unknown as string)
      console.log(created);
      this.sets = response;
      this.created = new Date(created);
    })*/
  }
}
