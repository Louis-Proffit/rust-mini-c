	.text
	.globl main
josephus:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq %rsi, -8(%rbp)
	call cercle
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
L2974:
	movq -16(%rbp), %rax
	movq -16(%rbp), %r10
	movq %r10, -32(%rbp)
	movq -32(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -24(%rbp)
	cmpq -24(%rbp), %rax
	setne %al
	testq %rax, %rax
	jz L2949
	movq $1, %r9
	movq %r9, %rcx
L2967:
	movq %r9, %rdi
	movq -8(%rbp), %rsi
	cmpq %rsi, %rdi
	setl %dil
	testq %rdi, %rdi
	jz L2955
	movq -16(%rbp), %rdx
	movq 8(%rdx), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r8
	movq $1, %rdx
	addq %rdx, %r9
	movq %r9, %rsi
	jmp L2967
L2955:
	movq -16(%rbp), %rdi
	call supprimer
	movq -16(%rbp), %rax
	movq 8(%rax), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rax
	jmp L2974
L2949:
	movq -16(%rbp), %rax
	movq 0(%rax), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
afficher:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -16(%rbp), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -16(%rbp), %r9
	movq 8(%r9), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rdi
L3070:
	movq -16(%rbp), %rdx
	movq -8(%rbp), %r8
	cmpq %r8, %rdx
	setne %dl
	testq %rdx, %rdx
	jz L3059
	movq -16(%rbp), %rcx
	movq 0(%rcx), %rdi
	call putchar
	movq -16(%rbp), %rdx
	movq 8(%rdx), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r9
	jmp L3070
L3059:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
cercle:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq %rdi, -8(%rbp)
	movq $1, %rdi
	call make
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq -24(%rbp), %rax
L3104:
	movq -24(%rbp), %rax
	movq $2, -32(%rbp)
	cmpq -32(%rbp), %rax
	setge %al
	testq %rax, %rax
	jz L3092
	movq -16(%rbp), %rdi
	movq -24(%rbp), %rsi
	call inserer_apres
	movq $1, %rax
	subq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	jmp L3104
L3092:
	movq -16(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
make:
	pushq %rbp
	movq %rsp, %rbp
	addq $-56, %rsp
	movq %rdi, -8(%rbp)
	movq $24, %rdi
	call malloc
	movq %rax, -56(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -40(%rbp)
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq -40(%rbp), %r11
	movq %r11, 0(%r10)
	movq %rax, -16(%rbp)
	movq %rax, -32(%rbp)
	movq -32(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 16(%r10)
	movq %rax, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	movq $7, %rdi
	movq $5, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $5, %rdi
	movq $5, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $5, %rdi
	movq $17, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $13, %rdi
	movq $2, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %r8
	movq -16(%rbp), %rax
	cqto
	idivq %r8
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -8(%rbp), %rdx
	movq $9, %rax
	cmpq %rax, %rdx
	setg %dl
	testq %rdx, %rdx
	jnz L3047
L3045:
	movq $48, %rdi
	movq -8(%rbp), %rcx
	movq $10, %rdx
	movq -16(%rbp), %rax
	imulq %rax, %rdx
	subq %rdx, %rcx
	addq %rcx, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L3047:
	movq -16(%rbp), %rdi
	call print_int
	jmp L3045
inserer_apres:
	pushq %rbp
	movq %rsp, %rbp
	addq $-88, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %rdi
	call make
	movq %rax, -88(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -80(%rbp)
	movq -80(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -64(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq -64(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -48(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 16(%r10)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
supprimer:
	pushq %rbp
	movq %rsp, %rbp
	addq $-48, %rsp
	movq %rdi, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -16(%rbp)
	movq -16(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -8(%rbp)
	movq 8(%rax), %rax
	movq -8(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

