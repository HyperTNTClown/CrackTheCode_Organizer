import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ApiService, PuzzleSet} from "../api.service";
import {CreatePuzzleSetComponent} from "./create-puzzle-set/create-puzzle-set.component";

@Component({
  selector: 'app-admin',
  standalone: true,
  imports: [CommonModule, CreatePuzzleSetComponent],
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
    this.api.fetch_sets().subscribe((response) => {
      console.log(response);
      let created= Date.parse(response[0].created as unknown as string)
      console.log(created);
      this.sets = response;
      this.created = new Date(created);
    })
  }
}
