import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { ENVIRONMENT } from '../../environments/environment.development';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ImageService {
  private http: HttpClient = inject(HttpClient);

  public getImages(page: number, limit: number): Observable<any> {
    return this.http.get(`${ENVIRONMENT.apiUrl}/images/${page}/${limit}`);
  }

  public getImage(id: string): Observable<any> {
    return this.http.get(`${ENVIRONMENT.apiUrl}/images/${id}`);
  }

  public uploadImage(formData: FormData): Observable<any> {
    return this.http.post(`${ENVIRONMENT.apiUrl}/upload`, formData);
  }
}
