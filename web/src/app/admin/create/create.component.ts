import {Component, Input, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {MatInputModule} from "@angular/material/input";
import {MarkdownComponent, MarkdownPipe, MarkdownService} from "ngx-markdown";
import {FormsModule} from "@angular/forms";
import {AngularMarkdownEditorModule} from "angular-markdown-editor";
import {ApiService} from "../../api.service";
import {MarkdownEditorComponent} from "../../markdown-editor/markdown-editor.component";
import {CdkDrag} from "@angular/cdk/drag-drop";

@Component({
             selector: 'app-create',
             standalone: true,
             imports: [CommonModule, MatInputModule, MarkdownComponent, FormsModule, AngularMarkdownEditorModule, MarkdownPipe, MarkdownEditorComponent, CdkDrag],
             providers: [ApiService],
             templateUrl: './create.component.html',
             styleUrl: './create.component.css',
             inputs: ['id', 'save']
           })
export class CreateComponent implements OnInit {

  constructor(private api: ApiService, private markdownService: MarkdownService) {
  }


  @Input() id: number | undefined;
  @Input({required: true}) save!: (name: string, description: string) => void;
  @Input({required: true}) creationTitle!: string;
  @Input({required: true}) ctype!: string;
  @Input({required: true}) saveText!: string;

  content: string = "";

  ngOnInit(): void {
  }

  protected readonly console = console;
  protected readonly document = document;
  title: string = "";
}
