import {Component, OnInit} from '@angular/core';
import { CommonModule } from '@angular/common';
import {ApiService, PuzzleSet} from "../../api.service";

@Component({
  selector: 'app-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './list.component.html',
  styleUrl: './list.component.css'
})
export class ListComponent implements OnInit {

  sets : PuzzleSet[] | undefined

  constructor(private api: ApiService) {
  }

  ngOnInit(): void {
    (async () => {
      this.sets = await this.api.fetch_sets()
    })()
  }



}
