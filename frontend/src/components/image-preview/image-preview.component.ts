import { ChangeDetectionStrategy, Component, Input } from '@angular/core';

@Component({
  selector: 'app-image-preview',
  standalone: true,
  imports: [],
  templateUrl: './image-preview.component.html',
  styleUrl: './image-preview.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ImagePreviewComponent {
  @Input() image!: any;
  public randomHeight: number = this.getRandomHeight();

  private getRandomHeight(): number {
    return Math.floor(Math.random() * (200 - 100 + 1)) + 100; // Random height between 100 and 200
  }

  public ngOnChange(): void {
    if (!this.image) {
      console.error('Image not found or invalid');
      return;
    }
  }
}
