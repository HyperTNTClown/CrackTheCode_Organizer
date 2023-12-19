import {Component, Input, OnInit} from '@angular/core';
import { CommonModule } from '@angular/common';
import {ApiService, Puzzle} from '../../api.service';
import {ActivatedRoute, Router} from '@angular/router';
import {Subscription} from 'rxjs';
import {MarkdownEditorComponent} from '../../markdown-editor/markdown-editor.component';
import {MarkdownComponent} from 'ngx-markdown';

@Component({
  selector: 'app-edit',
  standalone: true,
             imports: [CommonModule, MarkdownEditorComponent, MarkdownComponent],
  templateUrl: './edit.component.html',
  styleUrl: './edit.component.css',
  inputs: []
})
export class EditComponent implements OnInit {
  puzzleID!: number;
  puzzle: Puzzle | undefined;
  private sub!: Subscription;

  constructor(private api: ApiService, private route: ActivatedRoute, protected router: Router) {}

  ngOnInit(): void {
    this.sub = this.route.params.subscribe(params => {
      this.puzzleID = +params['id'];
    });
    (async () => {
      const puzzle = await this.api.fetch_puzzle(this.puzzleID)
      console.log(puzzle)
      this.puzzle = puzzle
    })()
  }

  save() {
    this.api.update_puzzle(this.puzzleID, this.puzzle!).then(r => console.log(r))
    this.router.navigate(['/admin/puzzle'])
  }
}
