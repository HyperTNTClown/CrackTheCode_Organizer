import {Component, EventEmitter, Input, Output} from '@angular/core';
import { CommonModule } from '@angular/common';
import {FormsModule} from "@angular/forms";

@Component({
  selector: 'app-markdown-editor',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './markdown-editor.component.html',
  styleUrl: './markdown-editor.component.css',
  outputs: ['content'],
  inputs: ['placeholder', 'content']
})
export class MarkdownEditorComponent {
  @Input() content!: string;
  @Output() contentChange = new EventEmitter<string>();
  @Input({required: true}) placeholder!: string;
  protected readonly HTMLTextAreaElement = HTMLTextAreaElement;

  emit($event: Event) {
    this.contentChange.emit(($event.target as HTMLTextAreaElement).value);
  }
}
