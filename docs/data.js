// Mock data for MedFiles SPA showcase
const mockData = {
  users: [
    {
      id: 1,
      email: "teste@medfiles.com",
      name: "Usuário Teste",
      created_at: "2024-01-01"
    }
  ],

  professionals: [
    {
      id: 1,
      name: "Dr. João Silva",
      specialty: "Cardiologia",
      crm: "12345-SP"
    },
    {
      id: 2,
      name: "Dra. Maria Santos",
      specialty: "Endocrinologia",
      crm: "23456-SP"
    },
    {
      id: 3,
      name: "Dr. Pedro Oliveira",
      specialty: "Psiquiatria",
      crm: "34567-SP"
    },
    {
      id: 4,
      name: "Dra. Ana Costa",
      specialty: "Ginecologia",
      crm: "45678-SP"
    },
    {
      id: 5,
      name: "Dr. Carlos Ferreira",
      specialty: "Neurologia",
      crm: "56789-SP"
    }
  ],

  medications: [
    {
      id: 1,
      name: "Losartana Potássica",
      active_ingredient: "Losartana",
      presentation: "Comprimido 50mg",
      current_dose: "50mg",
      status: "active"
    },
    {
      id: 2,
      name: "Metformina",
      active_ingredient: "Metformina",
      presentation: "Comprimido 500mg",
      current_dose: "500mg",
      status: "active"
    },
    {
      id: 3,
      name: "Sertralina",
      active_ingredient: "Sertralina",
      presentation: "Comprimido 50mg",
      current_dose: "50mg",
      status: "active"
    },
    {
      id: 4,
      name: "Omeprazol",
      active_ingredient: "Omeprazol",
      presentation: "Cápsula 20mg",
      current_dose: "20mg",
      status: "active"
    },
    {
      id: 5,
      name: "Aspirina",
      active_ingredient: "Ácido Acetilsalicílico",
      presentation: "Comprimido 100mg",
      current_dose: "100mg",
      status: "active"
    }
  ],

  prescriptions: [
    {
      id: 1,
      patient_name: "João Silva",
      patient_birth_date: "1985-03-15",
      patient_cpf: "123.456.789-00",
      date: "2024-09-01",
      professional_id: 1,
      notes: "Paciente com hipertensão controlada",
      medications: [
        {
          id: 1,
          name: "Losartana Potássica",
          dose: "50mg",
          quantity: "30 comprimidos",
          posology: "1 comprimido ao dia",
          presentation: "Comprimido 50mg"
        }
      ]
    },
    {
      id: 2,
      patient_name: "João Silva",
      patient_birth_date: "1985-03-15",
      patient_cpf: "123.456.789-00",
      date: "2024-08-15",
      professional_id: 2,
      notes: "Ajuste na dose de metformina",
      medications: [
        {
          id: 2,
          name: "Metformina",
          dose: "500mg",
          quantity: "60 comprimidos",
          posology: "1 comprimido 2x ao dia",
          presentation: "Comprimido 500mg"
        }
      ]
    },
    {
      id: 3,
      patient_name: "João Silva",
      patient_birth_date: "1985-03-15",
      patient_cpf: "123.456.789-00",
      date: "2024-07-20",
      professional_id: 3,
      notes: "Tratamento para depressão",
      medications: [
        {
          id: 3,
          name: "Sertralina",
          dose: "50mg",
          quantity: "30 comprimidos",
          posology: "1 comprimido ao dia",
          presentation: "Comprimido 50mg"
        }
      ]
    },
    {
      id: 4,
      patient_name: "João Silva",
      patient_birth_date: "1985-03-15",
      patient_cpf: "123.456.789-00",
      date: "2024-06-10",
      professional_id: 4,
      notes: "Controle de refluxo",
      medications: [
        {
          id: 4,
          name: "Omeprazol",
          dose: "20mg",
          quantity: "30 cápsulas",
          posology: "1 cápsula ao dia",
          presentation: "Cápsula 20mg"
        }
      ]
    },
    {
      id: 5,
      patient_name: "João Silva",
      patient_birth_date: "1985-03-15",
      patient_cpf: "123.456.789-00",
      date: "2024-05-05",
      professional_id: 1,
      notes: "Profilaxia cardiovascular",
      medications: [
        {
          id: 5,
          name: "Aspirina",
          dose: "100mg",
          quantity: "30 comprimidos",
          posology: "1 comprimido ao dia",
          presentation: "Comprimido 100mg"
        }
      ]
    }
  ],

  medicationChanges: [
    {
      id: 1,
      medication_id: 1,
      prescription_id: 1,
      old_dose: null,
      new_dose: "50mg",
      change_type: "new",
      date: "2024-09-01",
      reason: "Nova prescrição"
    },
    {
      id: 2,
      medication_id: 2,
      prescription_id: 2,
      old_dose: "850mg",
      new_dose: "500mg",
      change_type: "decrease",
      date: "2024-08-15",
      reason: "Ajuste por efeitos colaterais"
    },
    {
      id: 3,
      medication_id: 2,
      prescription_id: 2,
      old_dose: null,
      new_dose: "850mg",
      change_type: "new",
      date: "2024-07-01",
      reason: "Inicio do tratamento"
    }
  ]
};

// Helper functions for data access
function getUserByEmail(email) {
  return mockData.users.find(user => user.email === email);
}

function getPrescriptions() {
  return mockData.prescriptions.map(prescription => ({
    ...prescription,
    professional: mockData.professionals.find(prof => prof.id === prescription.professional_id)
  }));
}

function getActiveMedications() {
  return mockData.medications.filter(med => med.status === 'active');
}

function getMedicationHistory(medicationId) {
  const medication = mockData.medications.find(m => m.id === medicationId);
  const changes = mockData.medicationChanges
    .filter(change => change.medication_id === medicationId)
    .sort((a, b) => new Date(b.date) - new Date(a.date));

  const relatedPrescriptions = mockData.prescriptions
    .filter(p => p.medications.some(m => m.id === medicationId))
    .map(p => ({
      ...p,
      professional: mockData.professionals.find(prof => prof.id === p.professional_id)
    }));

  return {
    medication,
    changes,
    relatedPrescriptions
  };
}

function getPrescriptionById(id) {
  const prescription = mockData.prescriptions.find(p => p.id === id);
  if (!prescription) return null;

  return {
    ...prescription,
    professional: mockData.professionals.find(prof => prof.id === prescription.professional_id)
  };
}

function getStats() {
  return {
    totalPrescriptions: mockData.prescriptions.length,
    activeMedications: mockData.medications.filter(m => m.status === 'active').length,
    totalProfessionals: mockData.professionals.length,
    totalMedications: mockData.medications.length
  };
}