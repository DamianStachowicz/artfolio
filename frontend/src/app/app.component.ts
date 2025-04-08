import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MasonryLayoutComponent } from '../components/masonry-layout/masonry-layout.component';
import { ImagePreviewComponent } from '../components/image-preview/image-preview.component';
import { ImageService } from '../services/image.service';
import { map, Observable } from 'rxjs';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    RouterOutlet,
    MasonryLayoutComponent,
    MasonryLayoutComponent,
    ImagePreviewComponent,
  ],
  providers: [ImageService],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AppComponent {
  private imageService: ImageService = inject(ImageService);

  public loadImages(page: number, limit: number): Observable<any[]> {
    return this.imageService
      .getImages(page, limit)
      .pipe(map((response) => response.images));
  }
}
