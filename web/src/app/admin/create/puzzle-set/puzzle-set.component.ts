import {Component, Input} from '@angular/core';
import {CommonModule} from '@angular/common';
import {CreateComponent} from "../create.component";
import {ApiService} from "../../../api.service";

@Component({
             selector: 'app-puzzle-set',
             standalone: true,
             imports: [CommonModule, CreateComponent],
             templateUrl: './puzzle-set.component.html',
             styleUrl: './puzzle-set.component.css',
             inputs: ['id']
           })
export class PuzzleSetComponent {
  @Input() id!: number;

  constructor(private api: ApiService) {
  }

  save(name: string, description: string) {
    console.log("Saving")
    this.api.create_set(name, description).then((set) => {
      console.log("Saved")
      console.log(set)
    })
  }

}
