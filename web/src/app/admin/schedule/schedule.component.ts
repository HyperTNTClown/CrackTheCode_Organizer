import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {of} from "rxjs";

@Component({
  selector: 'app-schedule',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './schedule.component.html',
  styleUrl: './schedule.component.css'
})
export class ScheduleComponent {

  protected readonly of = of;
}
