// Este componente Header muestra el título de la aplicación y permite conectar o desconectar la billetera Stellar.
'use client';
import React, { useState } from "react";
import { useSorobanReact } from "stellar-react";

// Componente principal del encabezado
const Header: React.FC = () => {
  // Extraemos la dirección de la billetera, y las funciones para conectar y desconectar usando el hook de Soroban React
  const { address, connect, disconnect } = useSorobanReact();
  return (
    // Encabezado visual con estilos y título
    <header className="w-full flex justify-between items-center px-6 py-4 bg-black bg-opacity-80 border-b border-gray-800">
      <h1 className="text-2xl font-bold text-white">Stellar Wallet</h1>
      {/* Si no hay dirección, mostramos el botón para conectar la billetera */}
      {!address ? (
        <button
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded transition"
          onClick={connect}
        >
          Conectar billetera
        </button>
      // Si hay dirección, mostramos la dirección y el botón para desconectar */}
      ) : (
        <div className="flex items-center gap-4">
          {/* Mostramos la dirección de la billetera conectada */}
          <span className="text-gray-300 text-sm">{address}</span>
          <button
            className="bg-blue-400 hover:bg-blue-500 text-white px-4 py-2 rounded transition shadow-lg shadow-white/30"
            onClick={disconnect}
          >
            Desconectar
          </button>
        </div>
      )}
    </header>
  );
};

// Exportamos el componente para su uso en otras partes de la app
export default Header;