# adozioni-animali-piattaforma-web

Piattaforma web per facilitare l'adozione di animali da rifugi e associazioni.

## Overview

# Product Requirements Document (PRD)

## 1. Project Overview

**Project Name:** adozioni-animali-piattaforma-web

The objective of this project is to develop a modern web platform that connects animal shelters, associations, and individuals interested in adopting pets. The platform aims to streamline, simplify, and make the adoption process transparent, addressing the current fragmentation across websites, social networks, and phone contacts. By centralizing animal adoption listings and requests, the platform will increase the visibility of animals in shelters and facilitate efficient, secure communication between all parties.

---

## 2. Goals & Success Metrics

**Primary Goals:**
- Centralize and simplify the process of pet adoption in Italy.
- Increase the visibility of animals in shelters and associations.
- Provide a secure, transparent, and user-friendly experience for all stakeholders.

**Success Metrics:**
- **User Adoption:** At least 1000 registered users (adopters and shelters) within 6 months of launch.
- **Animal Listings:** At least 500 animals listed within 6 months.
- **Adoption Requests:** At least 200 adoption requests processed within 6 months.
- **Response Time:** 95% of API responses under 2 seconds.
- **User Satisfaction:** 80%+ positive feedback in user surveys.
- **Platform Uptime:** 99% uptime (excluding planned maintenance).

---

## 3. Target Users

### 3.1 Adopters
- Individuals seeking to adopt pets.
- Needs: Easy search/filtering, clear animal information, simple request process, updates on request status.

### 3.2 Shelters & Associations
- Organizations managing animals available for adoption.
- Needs: Efficient animal profile management, dashboard for adoption requests, communication tools, visibility.

### 3.3 Platform Administrators
- Staff responsible for moderation, user management, and platform integrity.
- Needs: Content moderation tools, user management, logs, ability to handle reports and approve/reject shelters.

---

## 4. Core Features

### 4.1 User Registration & Authentication
- **User Types:** Adopter, Shelter/Association, Administrator.
- **Registration Fields:**
  - **Adopter:** Name, Surname, Email, Password, Phone, City.
  - **Shelter:** Association Name, Contact Person, Email, Password, Phone, Address, Description, Verification Documents.
  - **Administrator:** Created by system admins; Name, Email, Credentials.
- **Authentication:** JWT-based, secure password storage, session management.
- **Legal Compliance:** Mandatory acceptance of privacy policy and terms of use.

### 4.2 Animal Profile Management
- **For Shelters:**
  - Create, edit, delete animal profiles.
  - Fields: Name, Species, Breed, Age, Sex, Size, Description, Status (Available, In Adoption, Adopted), Images.
- **Image Upload:** Local storage (dev), S3-compatible (prod).

### 4.3 Search & Filtering
- **Public Search:** Anyone can browse/search animals.
- **Filters:** Species, Breed, Age, Location (region/province/distance), Size, Status.
- **Sorting:** By recency, proximity, age, etc.

### 4.4 Adoption Requests
- **Flow:**
  1. Adopter selects animal, fills out request form.
  2. Request status: Inviata → In valutazione → Approvata/Rifiutata.
  3. Shelter receives notification, can view details, contact adopter, update status.
  4. Adopter receives notifications on every status change and can view request history.
- **Messaging:** Optional messaging thread per request between adopter and shelter.

### 4.5 Notifications
- **Email Notifications:** For key events (new request, status changes, new messages).
- **Notification Center:** In-app notification list, mark as read.
- **Push Notifications:** Out of scope for v1 (future enhancement).

### 4.6 Shelter Dashboard
- **Request Management:** View/manage all adoption requests, filter by status/animal.
- **Animal Management:** List, edit, archive animal profiles.
- **Communication:** Initiate/respond to messages with adopters.

### 4.7 Administrative Area
- **Moderation:** Approve/reject shelter registrations, review flagged content, edit/remove animal listings, suspend users.
- **Reporting:** View activity logs, manage user reports.
- **Content Validation:** Automatic checks for required fields, format, inappropriate content; manual review as needed.

### 4.8 Localization & Geographic Coverage
- **Languages:** Italian (v1), architecture ready for i18n.
- **Coverage:** Italy, with filters for region/province/distance.

---

## 5. Technical Architecture

### 5.1 Technology Stack

- **Frontend:** React, Bootstrap, Axios
- **Backend:** Node.js, Express
- **Database:** PostgreSQL
- **Authentication:** JWT (JSON Web Tokens)
- **Image Storage:** Local (dev), S3-compatible (prod)
- **Email:** SMTP (Mailtrap for dev, SendGrid/Amazon SES for prod)
- **Containerization:** Docker, Docker Compose
- **Version Control:** Git + GitHub

### 5.2 Key Components

- **API Layer:** RESTful endpoints for all core operations; stateless, token-based auth.
- **Frontend App:** SPA with responsive design, dynamic routing, state management.
- **Admin Panel:** Separate route/area with restricted access.
- **Notification Service:** Email delivery, in-app notification tracking.
- **Image Service:** Upload, validate, and serve images securely.

### 5.3 Data Models

#### User
| Field         | Type      | Notes                                 |
|---------------|-----------|---------------------------------------|
| id            | UUID      | Primary key                           |
| name          | String    |                                       |
| surname       | String    |                                       |
| email         | String    | Unique                                |
| password      | String    | Hashed                                |
| role          | Enum      | adopter, shelter, admin               |
| phone         | String    |                                       |
| city          | String    |                                       |
| address       | String    | Shelters only                         |
| description   | Text      | Shelters only                         |
| verification  | String    | Shelters only, document URL           |
| created_at    | DateTime  |                                       |

#### Animal
| Field         | Type      | Notes                                 |
|---------------|-----------|---------------------------------------|
| id            | UUID      | Primary key                           |
| name          | String    |                                       |
| species       | String    |                                       |
| breed         | String    |                                       |
| age           | Integer   |                                       |
| sex           | Enum      | M/F                                   |
| size          | Enum      | S/M/L/XL                              |
| description   | Text      |                                       |
| status        | Enum      | available, in_adoption, adopted       |
| shelter_id    | UUID      | FK to User                            |
| images        | Array     | URLs                                  |
| created_at    | DateTime  |                                       |

#### AdoptionRequest
| Field         | Type      | Notes                                 |
|---------------|-----------|---------------------------------------|
| id            | UUID      | Primary key                           |
| animal_id     | UUID      | FK to Animal                          |
| adopter_id    | UUID      | FK to User                            |
| status        | Enum      | sent, under_review, approved, rejected|
| message       | Text      | Optional                              |
| created_at    | DateTime  |                                       |

#### Notification
| Field         | Type      | Notes                                 |
|---------------|-----------|---------------------------------------|
| id            | UUID      | Primary key                           |
| user_id       | UUID      | Recipient                             |
| title         | String    |                                       |
| message       | Text      |                                       |
| type          | Enum      | e.g., request, status_change, message |
| read          | Boolean   |                                       |
| created_at    | DateTime  |                                       |

#### Message (optional, per request)
| Field         | Type      | Notes                                 |
|---------------|-----------|---------------------------------------|
| id            | UUID      | Primary key                           |
| request_id    | UUID      | FK to AdoptionRequest                 |
| sender_id     | UUID      | FK to User                            |
| content       | Text      |                                       |
| created_at    | DateTime  |                                       |

**Entity Relationships:**
- One Shelter (User) → Many Animals
- One Animal → Many AdoptionRequests
- One Adopter (User) → Many AdoptionRequests
- One AdoptionRequest → Many Messages
- One User → Many Notifications

### 5.4 API Endpoints (REST)

#### Authentication
- `POST /api/register`
- `POST /api/login`
- `POST /api/logout`
- `GET /api/user`

#### Animals
- `GET /api/animals`
- `GET /api/animals/:id`
- `POST /api/animals`
- `PUT /api/animals/:id`
- `DELETE /api/animals/:id`

#### Adoption Requests
- `POST /api/adoption-requests`
- `GET /api/adoption-requests`
- `GET /api/adoption-requests/:id`
- `PATCH /api/adoption-requests/:id/status`

#### Notifications
- `GET /api/notifications`
- `PATCH /api/notifications/:id/read`

#### Messaging (optional)
- `POST /api/adoption-requests/:id/messages`
- `GET /api/adoption-requests/:id/messages`

---

## 6. Non-Functional Requirements

- **Performance:** 95% of API responses < 2 seconds.
- **Responsiveness:** Fully responsive UI (desktop, tablet, mobile).
- **Security:**
  - JWT authentication, hashed passwords.
  - Data validation and sanitization.
  - GDPR-compliant data handling.
- **Scalability:** Modular architecture, Dockerized deployment, ready for horizontal scaling.
- **Extensibility:** Designed for future features (push notifications, multi-language, internationalization).
- **Reliability:** 99%+ uptime, robust error handling, logging.
- **Accessibility:** WCAG 2.1 AA compliance where feasible.

---

## 7. Out of Scope (v1)

- Push notifications (mobile/web).
- Mobile app (web only in v1).
- Payment processing or donations.
- Social network integrations.
- Public API for third-party integrations.
- Multi-country support (Italy only in v1).
- Advanced analytics/dashboard for shelters (basic stats only).
- Automated AI-based content moderation (manual + basic automated checks only).
- User reviews/ratings.

---

## 8. Open Questions

- Should there be a public-facing landing page for non-logged-in users, or is registration required to browse animals?
- What is the minimum required documentation for shelter verification (file types, review process)?
- Should messaging between adopter and shelter be real-time (WebSocket) or asynchronous (REST only) in v1?
- What is the retention policy for user data and adoption request history?
- Are there specific accessibility requirements beyond standard WCAG compliance?
- Should animal profiles support video uploads in addition to images (future)?
- Is there a need for a reporting/flagging mechanism for inappropriate animal listings or user behavior in v1?

---

**End of PRD**