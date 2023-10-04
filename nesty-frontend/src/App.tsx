import styled, { ThemeProvider } from "styled-components";
import { GlobalStyle } from "./components/globaStyles";
import { theme } from "./components/theme";
import Register from "./components/Register";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

const StyledApp = styled.div`
  width: 400px;
  height: 320px;
  background-color: ${({ theme }) => theme.colors.primary};
  color: white;
  padding: 5px;
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  display: flex;

  #registers {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  p {
    margin: 0;
  }

  #stack {
    height: 100%;
    flex-grow: 1;
    overflow-y: scroll;
    border-radius: 5px;

    &::-webkit-scrollbar {
      width: 5px;
      border-radius: 2px;
    }

    &::-webkit-scrollbar-track-piece {
      background-color: transparent;
    }

    &::-webkit-scrollbar-thumb {
      background-color: ${({ theme }) => theme.colors.secondary};
      border-radius: 2px;
    }

    table {
      width: 100%;

      tr {
        &:nth-of-type(odd) {
          background-color: #1d999d;
          font-weight: bold;
        }

        &:nth-of-type(even) {
          background-color: white;
          color: black;
          font-weight: bold;
        }
      }

      td {
        padding: 5px;
        border-radius: 5px;
      }

      thead {
        td {
          background-color: #0c2c57;
        }
      }
    }
  }

  #buttons {
    display: flex;
    flex-grow: 1;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }
`;

const StyledButton = styled.button`
  height: 2rem;
  border-radius: 5px;
  border: none;
  font-weight: bolder;
  width: 100%;
`;

export interface IRegister {
  name: string;
  value: string;
}

export interface IMemoryCell {
  address: number;
  value: number;
}

function App() {
  const registers: IRegister[] = [
    { name: "A", value: "0x00" },
    { name: "X", value: "0x00" },
    { name: "Y", value: "0x00" },
    { name: "SP", value: "0x00" },
    { name: "PC", value: "0x00" },
    { name: "Status Flags", value: "00000000" },
  ];

  const [stack, setStack] = useState<IMemoryCell[]>([]);
  useEffect(() => {
    appWindow.listen("cpu-update", async (event) => {
      console.log(event.payload);
    });

    let s = [];
    for (let i = 0xff; i >= 0; i--) {
      s.push({ address: i, value: 0 });
    }
    setStack(s);
  }, []);

  return (
    <ThemeProvider theme={theme}>
      <GlobalStyle />
      <StyledApp>
        <div id="registers">
          {registers.map((register) => (
            <Register register={register} key={register.name} />
          ))}
          <div id="buttons">
            <StyledButton>PAUSE</StyledButton>
            <StyledButton>RESET</StyledButton>
          </div>
        </div>
        <div id="stack">
          <table>
            <thead>
              <tr>
                <td>Address</td>
                <td>Value</td>
              </tr>
            </thead>
            <tbody>
              {stack.map((memcell) => (
                <tr key={memcell.address}>
                  <td>0x{memcell.address.toString(16).toUpperCase()}</td>
                  <td>0x{memcell.value.toString(16).toUpperCase()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </StyledApp>
    </ThemeProvider>
  );
}

export default App;
