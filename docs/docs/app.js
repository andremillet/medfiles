// MedFiles SPA Application
class MedFilesApp {
    constructor() {
        this.currentUser = null;
        this.currentPrescriptionId = null;
        this.currentMedicationId = null;
        this.init();
    }

    init() {
        this.setupRouting();
        this.setupEventListeners();
        this.checkAuthentication();
    }

    setupRouting() {
        // Handle browser back/forward buttons
        window.addEventListener('popstate', (e) => {
            if (e.state && e.state.page) {
                this.navigateToPage(e.state.page, e.state.params);
            }
        });

        // Handle initial load
        const urlParams = new URLSearchParams(window.location.search);
        const page = urlParams.get('page') || 'login';
        const params = {};
        urlParams.forEach((value, key) => {
            if (key !== 'page') params[key] = value;
        });

        this.navigateToPage(page, params);
    }

    setupEventListeners() {
        // Login form
        const loginForm = document.getElementById('loginForm');
        if (loginForm) {
            loginForm.addEventListener('submit', this.handleLogin.bind(this));
        }

        // Modal close buttons
        document.querySelectorAll('.modal-close').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const modal = e.target.closest('.modal');
                this.closeModal(modal.id);
            });
        });

        // Close modal when clicking outside
        document.querySelectorAll('.modal').forEach(modal => {
            modal.addEventListener('click', (e) => {
                if (e.target === modal) {
                    this.closeModal(modal.id);
                }
            });
        });

        // Upload modal file handling
        const modalFileInput = document.getElementById('modalFileInput');
        const modalUploadArea = document.getElementById('modalUploadArea');

        if (modalUploadArea) {
            modalUploadArea.addEventListener('click', () => {
                modalFileInput.click();
            });

            modalFileInput.addEventListener('change', (e) => {
                if (e.target.files.length > 0) {
                    this.handleFileUpload(e.target.files[0]);
                }
            });

            // Drag and drop
            modalUploadArea.addEventListener('dragover', (e) => {
                e.preventDefault();
                modalUploadArea.classList.add('dragover');
            });

            modalUploadArea.addEventListener('dragleave', () => {
                modalUploadArea.classList.remove('dragover');
            });

            modalUploadArea.addEventListener('drop', (e) => {
                e.preventDefault();
                modalUploadArea.classList.remove('dragover');
                const files = e.dataTransfer.files;
                if (files.length > 0) {
                    this.handleFileUpload(files[0]);
                }
            });
        }
    }

    navigateToPage(page, params = {}) {
        // Hide all sections
        document.querySelectorAll('.page-section').forEach(section => {
            section.classList.add('hidden');
        });

        // Show target section
        const targetSection = document.getElementById(`${page}-section`);
        if (targetSection) {
            targetSection.classList.remove('hidden');

            // Update URL without reloading
            const url = new URL(window.location);
            url.searchParams.set('page', page);
            Object.keys(params).forEach(key => {
                url.searchParams.set(key, params[key]);
            });
            window.history.pushState({ page, params }, '', url);

            // Load page-specific data
            this.loadPageData(page, params);
        }
    }

    loadPageData(page, params) {
        switch (page) {
            case 'dashboard':
                this.loadDashboardData();
                break;
            case 'prescription-detail':
                this.currentPrescriptionId = params.id;
                this.loadPrescriptionDetail(params.id);
                break;
            case 'medication-history':
                this.currentMedicationId = params.id;
                this.loadMedicationHistory(params.id);
                break;
        }
    }

    checkAuthentication() {
        // Check if user is logged in (simulate with localStorage)
        const user = localStorage.getItem('medfiles_user');
        if (user) {
            this.currentUser = JSON.parse(user);
            this.navigateToPage('dashboard');
        } else {
            this.navigateToPage('login');
        }
    }

    handleLogin(event) {
        event.preventDefault();

        const email = document.getElementById('email').value;
        const password = document.getElementById('password').value;

        // Simple authentication simulation
        const user = getUserByEmail(email);
        if (user && password === 'teste123') {
            this.currentUser = user;
            localStorage.setItem('medfiles_user', JSON.stringify(user));
            this.navigateToPage('dashboard');
        } else {
            this.showMessage('Credenciais inválidas. Use teste@medfiles.com / teste123', 'error');
        }
    }

    logout() {
        this.currentUser = null;
        localStorage.removeItem('medfiles_user');
        this.navigateToPage('login');
    }

    loadDashboardData() {
        const stats = getStats();
        this.updateStats(stats);

        const medications = getActiveMedications();
        this.updateActiveMedications(medications);

        const prescriptions = getPrescriptions();
        this.updateRecentPrescriptions(prescriptions);
    }

    updateStats(stats) {
        document.getElementById('totalPrescriptions').textContent = stats.totalPrescriptions;
        document.getElementById('activeMedications').textContent = stats.activeMedications;
        document.getElementById('totalProfessionals').textContent = stats.totalProfessionals;
        document.getElementById('totalMedications').textContent = stats.totalMedications;
    }

    updateActiveMedications(medications) {
        const container = document.getElementById('activeMedicationsGrid');

        if (medications.length === 0) {
            container.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-pills"></i>
                    <h3>Nenhuma medicação ativa</h3>
                    <p>Adicione prescrições para ver suas medicações aqui.</p>
                </div>
            `;
            return;
        }

        container.innerHTML = medications.map(med => `
            <div class="medication-card" onclick="app.viewMedicationHistory(${med.id})">
                <div class="medication-header">
                    <div>
                        <div class="medication-name">${med.name}</div>
                        <div class="medication-dose">${med.current_dose}</div>
                    </div>
                    <div class="medication-actions">
                        <button class="history-btn" onclick="event.stopPropagation(); app.viewMedicationHistory(${med.id})">
                            <i class="fas fa-history"></i>
                        </button>
                    </div>
                </div>
                <div class="medication-details">
                    <div class="medication-detail">
                        <label>Apresentação:</label>
                        <span>${med.presentation}</span>
                    </div>
                </div>
            </div>
        `).join('');
    }

    updateRecentPrescriptions(prescriptions) {
        const container = document.getElementById('recentPrescriptions');

        if (prescriptions.length === 0) {
            container.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-file-prescription"></i>
                    <h3>Nenhuma prescrição encontrada</h3>
                    <p>Adicione sua primeira prescrição para começar.</p>
                </div>
            `;
            return;
        }

        // Sort by date (most recent first) and take first 5
        const recentPrescriptions = prescriptions
            .sort((a, b) => new Date(b.date) - new Date(a.date))
            .slice(0, 5);

        container.innerHTML = recentPrescriptions.map(presc => {
            const date = new Date(presc.date).toLocaleDateString('pt-BR');
            const medications = presc.medications || [];

            return `
                <div class="prescription-item" onclick="app.viewPrescriptionDetail(${presc.id})">
                    <div class="prescription-header">
                        <div class="prescription-date">${date}</div>
                        <div class="prescription-professional">${presc.professional.name}</div>
                    </div>
                    <div class="prescription-medications">
                        ${medications.map(med => `
                            <span class="medication-tag">${med.name} ${med.dose}</span>
                        `).join('')}
                    </div>
                </div>
            `;
        }).join('');
    }

    loadPrescriptionDetail(prescriptionId) {
        const prescription = getPrescriptionById(prescriptionId);
        if (!prescription) {
            this.showMessage('Prescrição não encontrada', 'error');
            return;
        }

        // Update header
        document.getElementById('prescriptionDate').textContent = new Date(prescription.date).toLocaleDateString('pt-BR');
        document.getElementById('professionalName').textContent = prescription.professional.name;

        // Update patient info
        document.getElementById('patientName').textContent = prescription.patient_name;
        document.getElementById('patientBirthDate').textContent = new Date(prescription.patient_birth_date).toLocaleDateString('pt-BR');
        document.getElementById('patientCPF').textContent = prescription.patient_cpf;

        // Update medications
        const medicationsList = document.getElementById('medicationsList');
        medicationsList.innerHTML = prescription.medications.map(med => `
            <div class="medication-item">
                <h3><i class="fas fa-pills"></i> ${med.name}</h3>
                <div class="medication-details">
                    <div class="medication-detail">
                        <label>Dose:</label>
                        <span>${med.dose}</span>
                    </div>
                    <div class="medication-detail">
                        <label>Quantidade:</label>
                        <span>${med.quantity}</span>
                    </div>
                    <div class="medication-detail">
                        <label>Posologia:</label>
                        <span>${med.posology}</span>
                    </div>
                    <div class="medication-detail">
                        <label>Apresentação:</label>
                        <span>${med.presentation}</span>
                    </div>
                </div>
            </div>
        `).join('');

        // Update professional info
        document.getElementById('profName').textContent = prescription.professional.name;
        document.getElementById('profSpecialty').textContent = prescription.professional.specialty;
        document.getElementById('profCRM').textContent = prescription.professional.crm;

        // Update notes
        document.getElementById('prescriptionNotes').innerHTML = `<p>${prescription.notes || 'Nenhuma observação adicional.'}</p>`;

        // Update footer
        document.getElementById('generatedDate').textContent = new Date().toLocaleDateString('pt-BR');
    }

    loadMedicationHistory(medicationId) {
        const historyData = getMedicationHistory(medicationId);
        if (!historyData) {
            this.showMessage('Histórico não encontrado', 'error');
            return;
        }

        const { medication, changes, relatedPrescriptions } = historyData;

        // Update header
        document.getElementById('medicationName').textContent = medication.name;

        // Update current status
        const currentStatus = document.getElementById('currentStatus');
        currentStatus.innerHTML = `
            <h3><i class="fas fa-info-circle"></i> ${medication.name}</h3>
            <div class="status-details">
                <div class="status-detail">
                    <label>Dose Atual:</label>
                    <span>${medication.current_dose}</span>
                </div>
                <div class="status-detail">
                    <label>Apresentação:</label>
                    <span>${medication.presentation}</span>
                </div>
                <div class="status-detail">
                    <label>Status:</label>
                    <span>${medication.status === 'active' ? 'Ativo' : 'Inativo'}</span>
                </div>
            </div>
        `;

        // Update dose chart (simplified)
        this.updateDoseChart(changes);

        // Update change timeline
        const changeTimeline = document.getElementById('changeTimeline');
        if (changes.length === 0) {
            changeTimeline.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-history"></i>
                    <h3>Nenhuma mudança registrada</h3>
                    <p>Este medicamento não teve alterações de dose.</p>
                </div>
            `;
        } else {
            changeTimeline.innerHTML = changes.map(change => `
                <div class="timeline-item">
                    <div class="timeline-icon">
                        <i class="fas fa-arrow-${change.change_type === 'decrease' ? 'down' : 'up'}"></i>
                    </div>
                    <div class="timeline-content">
                        <h4>${this.formatChangeDescription(change)}</h4>
                        <p>${change.reason || 'Motivo não informado'}</p>
                        <div class="timeline-date">${new Date(change.date).toLocaleDateString('pt-BR')}</div>
                    </div>
                </div>
            `).join('');
        }

        // Update related prescriptions
        const relatedPrescriptionsContainer = document.getElementById('relatedPrescriptions');
        if (relatedPrescriptions.length === 0) {
            relatedPrescriptionsContainer.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-file-prescription"></i>
                    <h3>Nenhuma prescrição relacionada</h3>
                    <p>Não foram encontradas prescrições para este medicamento.</p>
                </div>
            `;
        } else {
            relatedPrescriptionsContainer.innerHTML = relatedPrescriptions.map(presc => `
                <div class="prescription-item" onclick="app.viewPrescriptionDetail(${presc.id})">
                    <div class="prescription-header">
                        <div class="prescription-date">${new Date(presc.date).toLocaleDateString('pt-BR')}</div>
                        <div class="prescription-professional">${presc.professional.name}</div>
                    </div>
                    <div class="prescription-medications">
                        ${presc.medications.map(med => `
                            <span class="medication-tag">${med.name} ${med.dose}</span>
                        `).join('')}
                    </div>
                </div>
            `).join('');
        }
    }

    updateDoseChart(changes) {
        const ctx = document.getElementById('doseChart');
        if (!ctx) return;

        // Prepare data for chart
        const labels = changes.map(change => new Date(change.date).toLocaleDateString('pt-BR'));
        const doses = changes.map(change => parseInt(change.new_dose) || 0);

        new Chart(ctx, {
            type: 'line',
            data: {
                labels: labels,
                datasets: [{
                    label: 'Dose (mg)',
                    data: doses,
                    borderColor: '#667eea',
                    backgroundColor: 'rgba(102, 126, 234, 0.1)',
                    tension: 0.4
                }]
            },
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        display: false
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true
                    }
                }
            }
        });
    }

    formatChangeDescription(change) {
        switch (change.change_type) {
            case 'new':
                return `Nova prescrição: ${change.new_dose}`;
            case 'decrease':
                return `Dose reduzida: ${change.old_dose} → ${change.new_dose}`;
            case 'increase':
                return `Dose aumentada: ${change.old_dose} → ${change.new_dose}`;
            default:
                return `Mudança registrada`;
        }
    }

    viewPrescriptionDetail(prescriptionId) {
        this.navigateToPage('prescription-detail', { id: prescriptionId });
    }

    viewMedicationHistory(medicationId) {
        this.navigateToPage('medication-history', { id: medicationId });
    }

    goBack() {
        window.history.back();
    }

    showModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.add('show');
        }
    }

    closeModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.remove('show');
        }
    }

    handleFileUpload(file) {
        // Simulate file upload
        setTimeout(() => {
            this.showUploadResult('Arquivo processado com sucesso! (simulação)', 'success');
            setTimeout(() => {
                this.closeModal('uploadModal');
                this.loadDashboardData(); // Refresh data
            }, 2000);
        }, 1000);
    }

    showUploadResult(message, type) {
        const resultDiv = document.getElementById('uploadResult');
        resultDiv.style.display = 'block';
        resultDiv.className = `upload-result ${type}`;
        resultDiv.innerHTML = `
            <i class="fas fa-${type === 'success' ? 'check-circle' : 'exclamation-triangle'}"></i>
            ${message}
        `;
    }

    showConsolidatedView() {
        const medications = getActiveMedications();
        const container = document.getElementById('consolidatedMedications');

        if (medications.length === 0) {
            container.innerHTML = `
                <div class="empty-state">
                    <i class="fas fa-pills"></i>
                    <h3>Nenhuma medicação encontrada</h3>
                    <p>Adicione prescrições para ver a visão consolidada.</p>
                </div>
            `;
        } else {
            // Group medications by name
            const medicationGroups = this.groupMedicationsByName(medications);

            container.innerHTML = `
                <div class="consolidated-header">
                    <h4><i class="fas fa-pills"></i> Prescrição Consolidada</h4>
                    <p>Visão unificada de todas as suas medicações ativas.</p>
                </div>
                <div class="consolidated-stats">
                    <div class="stat-item">
                        <span class="stat-number">${medications.length}</span>
                        <span class="stat-label">Medicações Ativas</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-number">${Object.keys(medicationGroups).length}</span>
                        <span class="stat-label">Medicamentos Únicos</span>
                    </div>
                </div>
                <div class="consolidated-list">
                    ${Object.entries(medicationGroups).map(([name, meds]) => `
                        <div class="consolidated-group">
                            <div class="group-header">
                                <h5>${name}</h5>
                                <div class="group-badges">
                                    <span class="badge stable">Ativo</span>
                                    <span class="badge prescriptions">${meds.length} prescrição(ões)</span>
                                </div>
                            </div>
                            <div class="group-content">
                                <div class="current-medication">
                                    <div class="medication-info">
                                        <div class="medication-primary">
                                            <span class="dose">${meds[0].current_dose}</span>
                                            <span class="posology">Conforme prescrição</span>
                                        </div>
                                        <div class="medication-secondary">
                                            <div>Apresentação: ${meds[0].presentation}</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    `).join('')}
                </div>
            `;
        }

        this.showModal('consolidatedModal');
    }

    groupMedicationsByName(medications) {
        const groups = {};
        medications.forEach(med => {
            if (!groups[med.name]) {
                groups[med.name] = [];
            }
            groups[med.name].push(med);
        });
        return groups;
    }

    showMessage(message, type) {
        // Remove existing messages
        const existingMessage = document.querySelector('.error-message, .success-message');
        if (existingMessage) {
            existingMessage.remove();
        }

        const messageDiv = document.createElement('div');
        messageDiv.className = `${type}-message`;
        messageDiv.innerHTML = `
            <i class="fas fa-${type === 'error' ? 'exclamation-triangle' : 'check-circle'}"></i>
            ${message}
        `;

        // Add to appropriate container
        const container = document.querySelector('.login-form') || document.body;
        container.appendChild(messageDiv);

        // Auto-remove success messages
        if (type === 'success') {
            setTimeout(() => {
                messageDiv.remove();
            }, 3000);
        }
    }
}

// Global functions for HTML onclick handlers
function showUploadModal() {
    app.showModal('uploadModal');
}

function showConsolidatedView() {
    app.showConsolidatedView();
}

function showPharmacyModal() {
    app.showModal('pharmacyModal');
}

function showReports() {
    app.showModal('reportsModal');
}

function closeModal(modalId) {
    app.closeModal(modalId);
}

function logout() {
    app.logout();
}

function goBack() {
    app.goBack();
}

// Initialize app when DOM is loaded
let app;
document.addEventListener('DOMContentLoaded', () => {
    app = new MedFilesApp();
});