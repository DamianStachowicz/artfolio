import {
  Component,
  Input,
  OnDestroy,
  AfterViewInit,
  ChangeDetectorRef,
  inject,
  ContentChild,
  AfterContentInit,
  TemplateRef,
  ChangeDetectionStrategy,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { Observable, Subject, takeUntil } from 'rxjs';

@Component({
  selector: 'app-masonry-layout',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './masonry-layout.component.html',
  styleUrl: './masonry-layout.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MasonryLayoutComponent
  implements AfterContentInit, AfterViewInit, OnDestroy
{
  @Input() loadData!: (page: number, limit: number) => Observable<any[]>;
  @Input() pageSize: number = 10;
  @ContentChild('masonryItem') masonryItem!: TemplateRef<any>;

  private cdr: ChangeDetectorRef = inject(ChangeDetectorRef);
  private currentPage: number = 1;
  public items: any[] = [];
  private loading: boolean = false;
  private destroy$: Subject<void> = new Subject<void>();

  public ngAfterContentInit() {
    const item =
      this.masonryItem.elementRef && this.masonryItem.elementRef.nativeElement;

    if (!item || !item.item) {
      console.error('Masonry item not found or invalid');
      return;
    }
  }

  public ngAfterViewInit(): void {
    this.loadInitialData();
  }

  public ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  private loadInitialData(): void {
    this.loadMoreData();
  }

  private loadMoreData(): void {
    if (this.loading) return;
    if (!this.loadData) {
      console.error('loadData function is not provided');
      return;
    }

    this.loading = true;
    this.loadData(this.currentPage, this.pageSize)
      .pipe(takeUntil(this.destroy$))
      .subscribe((data) => {
        this.items = [...this.items, ...data];
        this.currentPage++;
        this.loading = false;
        console.log({ items: this.items, currentPage: this.currentPage });
        this.cdr.detectChanges();
      });
  }

  public onScroll(event: any): void {
    const bottom =
      event.target.scrollHeight ===
      event.target.scrollTop + event.target.clientHeight;
    if (bottom) {
      this.loadMoreData();
    }
  }
}
