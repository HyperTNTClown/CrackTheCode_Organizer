import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {CreateComponent} from "../create.component";

@Component({
  selector: 'app-puzzle',
  standalone: true,
             imports: [CommonModule, CreateComponent],
  templateUrl: './puzzle.component.html',
  styleUrl: './puzzle.component.css'
})
export class PuzzleComponent {
  save() {
    throw new Error('Method not implemented.');
  }

}
