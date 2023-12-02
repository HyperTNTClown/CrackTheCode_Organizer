import {Component, Input, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {MatInputModule} from "@angular/material/input";
import {MarkdownComponent, MarkdownPipe, MarkdownService} from "ngx-markdown";
import {FormsModule} from "@angular/forms";
import {AngularMarkdownEditorModule, EditorOption} from "angular-markdown-editor";
import {ApiService} from "../../api.service";
import {MarkdownEditorComponent} from "../../markdown-editor/markdown-editor.component";

@Component({
  selector: 'app-create-puzzle-set',
  standalone: true,
  imports: [CommonModule, MatInputModule, MarkdownComponent, FormsModule, AngularMarkdownEditorModule, MarkdownPipe, MarkdownEditorComponent],
  providers: [ApiService],
  templateUrl: './create-puzzle-set.component.html',
  styleUrl: './create-puzzle-set.component.css',
  inputs: ['id']
})
export class CreatePuzzleSetComponent implements OnInit {

  constructor(private api: ApiService, private markdownService: MarkdownService) {
  }


  @Input() id: number | undefined;

  content: string = "";

  // TODO: Implement your own Markdown editor to replace this one.

  ngOnInit(): void {
  }

  protected readonly console = console;

  protected readonly document = document;

  private save() {
    console.log(this.content)
  }
}
