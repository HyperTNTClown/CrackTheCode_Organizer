import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {CreateComponent} from "../create.component";
import {ApiService} from '../../../api.service';

@Component({
  selector: 'app-puzzle',
  standalone: true,
             imports: [CommonModule, CreateComponent],
  templateUrl: './puzzle.component.html',
  styleUrl: './puzzle.component.css'
})
export class PuzzleComponent {

  constructor(private api: ApiService) {}
  save(name: string, description: string) {
    this.api.save_puzzle(name, description).then(r => console.log(r))
  }

}
