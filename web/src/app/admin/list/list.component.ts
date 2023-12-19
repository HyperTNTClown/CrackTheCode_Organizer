import {Component, OnInit} from '@angular/core';
import { CommonModule } from '@angular/common';
import {ApiService, Puzzle, PuzzleSet} from '../../api.service';
import {of} from 'rxjs';
import {MarkdownComponent} from 'ngx-markdown';
import {Router} from '@angular/router';
@Component({
  selector: 'app-list',
  standalone: true,
             imports: [CommonModule, MarkdownComponent],
  templateUrl: './list.component.html',
  styleUrl: './list.component.css'
})
export class ListComponent implements OnInit {

  sets! : Puzzle[]
  currentDeleteId: number | undefined;

  constructor(private api: ApiService, private router: Router) {
  }

  ngOnInit(): void {
    (async () => {
      this.sets = await this.api.fetch_puzzles()
    })()
  }
  openModal(id: number) {
    const dialog = document.getElementById('dialog'+id) as HTMLDialogElement;
    dialog.showModal();
    dialog.show()
    dialog.showPopover()
  }

  deletePuzzle(id: number) {
    this.currentDeleteId = id;
    (document.getElementById('areyousure') as HTMLDialogElement).showModal();
  }

  finalizeDeletion() {
    (document.getElementById('areyousure') as HTMLDialogElement).close()
    if (this.currentDeleteId == undefined) {
      return
    }
    this.api.delete_puzzle(this.currentDeleteId).then(r => console.log(r))
    document.getElementById('dialog'+this.currentDeleteId)?.remove()
    this.sets = this.sets.filter(s => s.id != this.currentDeleteId)
  }

  editPuzzle(id: number) {
    this.router.navigate(['/admin/puzzle-edit', id])
  }
}
