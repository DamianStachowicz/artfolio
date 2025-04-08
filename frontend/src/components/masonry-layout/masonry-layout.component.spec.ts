import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MasonryLayoutComponent } from './masonry-layout.component';

describe('MasonryLayoutComponent', () => {
  let component: MasonryLayoutComponent;
  let fixture: ComponentFixture<MasonryLayoutComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MasonryLayoutComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MasonryLayoutComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
