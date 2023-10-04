import styled from "styled-components";
import { IRegister } from "../App";

const StyledRegister = styled.div`
  background-color: ${({ theme }) => theme.colors.secondary};
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 5px 0 5px 15px;
  height: 24px;
  gap: 1rem;
  border-radius: 5px;
  font-weight: 700;

  p {
    &:last-of-type {
      background-color: white;
      color: black;
      padding: 2px;
      margin-right: 2px;
      border-radius: 5px;
      font-weight: 700;
    }
  }
`;

function Register({ register }: { register: IRegister }) {
  return (
    <StyledRegister>
      <p>{register.name} :</p> <p>{register.value}</p>
    </StyledRegister>
  );
}

export default Register;
